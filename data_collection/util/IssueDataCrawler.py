import json
import os

import requests
import logging


class IssueDataCrawler:
    def __init__(self, output_path, token):
        self.token = token
        self.header = {
            'X-GitHub-Api-Version': '2022-11-28',
            'Authorization': "Bearer " + self.token
        }
        self.output_data_path = os.path.dirname(output_path).join("issues.txt")

    def fetch_issues_data(self):
        base_url = 'https://api.github.com/repos/rust-lang/rust/issues'

        # 'labels': 'A-trait-system,A-lifetimes,A-inference,A-type-system,A-borrow-checker,A-closures,A-impl-trait,A-MIR,A-mir-opt,A-trait-objects,A-auto-traits,A-mir-opt-inlining,A-implied-bounds,A-HIR',
        params = {
            'state': 'closed',
            'per_page': 100,
            'page': 1,
            # 'labels': 'C-bug',
            'since': '2022-01-01T00:00:00Z',
        }
        issues = []
        target_labels = ['A-trait-system', 'A-lifetimes', 'A-inference', 'A-type-system', 'A-borrow-checker',
                        'A-closures', 'A-impl-trait', 'A-MIR', 'A-mir-opt', 'A-trait-objects', 'A-auto-traits',
                        'A-mir-opt-inlining', 'A-implied-bounds', 'A-HIR',
                        'A-coercions', 'A-coherence', 'A-coinduction', 'A-const-generics', 'A-DSTs', 'A-mir-opt-GVN', 'A-mir-opt-nrvo', 'A-stable-MIR', 'A-THIR', 'A-zst']

        while True:
            labeled_issues = []
            response = requests.get(base_url, headers=self.header, params=params)
            if response.status_code != 200:
                logging.error(f"Failed to fetch issues. Status code: {response.status_code}")
                break

            fetched_issues = response.json()

            for issue in fetched_issues:
                issue_labels = set(label['name'] for label in issue.get('labels', []))
                # print(issue_labels)
                if any(label in issue_labels for label in target_labels):
                    labeled_issues.append(issue)
            # labeled_issues = [issue for issue in fetched_issues if any(label in {label['name'] for label in issue.get('labels', [])} for label in target_labels)]

            filtered_issues = [issue for issue in labeled_issues if issue.get('pull_request') is None]

            issues.extend(filtered_issues)
            logging.info(f"Fetched {len(issues)} issue data.")

            link_header = response.headers.get('Link', '')
            if 'rel="next"' not in link_header:
                break

            params['page'] += 1

        return issues

    def write_issues_data_to_file(self, issues):
        with open(self.output_data_path, 'w') as file:
            json.dump(issues, file)

        logging.info(f"Issues written to {self.output_data_path}")

    def crawl_issues(self):
        issues = self.fetch_issues_data()
        self.write_issues_data_to_file(issues)
