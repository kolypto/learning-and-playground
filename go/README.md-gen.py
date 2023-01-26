#!/usr/bin/env python

import sys
import pathlib

def main(files: list[str], outfile_path: str):
    with open(outfile_path, 'wt') as md:
        md.write('# Go\n\n')

        prev_folder = None
        for file_path in files:
            file = pathlib.Path(file_path)
            
            # Folder header
            this_folder = str(file.parent)
            if this_folder != prev_folder:
                md.write(f'\n\n\n\n# {this_folder}')
                prev_folder = this_folder

            # File header
            md.write(f'\n\n## {file}\n')

            # File contents
            contents = file.read_text()
            # Markdowns: as is
            if file.suffix == '.md':
                md.write(contents)
            else:
                md_highlight = {
                    '.py': 'python',
                    '.go': 'go',
                    '.sql': 'sql',
                    '.sh': 'bash',
                    '.mod': '',
                    '.toml': 'toml',
                    '.yaml': 'yaml',
                    '.proto': 'protobuf',
                    '.fbs': 'flatbuffers',
                }[file.suffix]

                md.write(f'```{md_highlight}\n{contents}\n```\n')




files = sys.argv[1:]
main(files, 'README.md')
