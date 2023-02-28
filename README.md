# LessPass CSV to RockPass JSON Converter

This is a simple command-line tool that converts a LessPass CSV file into a RockPass JSON file that can be imported within a RockPass server. The tool takes the path of the input CSV file and the path of the output JSON file as command-line arguments, and converts the data in the CSV file into JSON format, writing the result to the output file.

## Prerequisites

To use this tool, you need to have Rust installed on your system. You can download Rust from the official website here.

## Usage

To use the tool, open a terminal and navigate to the directory containing the csv-to-json binary. Then, run the following command:

```bash
$ ./csv-to-json <input-csv> <output-json>
```

Here, `<input-csv>` is the path to the input CSV file, and `<output-json>` is the path to the output JSON file. Make sure to replace these placeholders with the actual file paths on your system.

For example, if you have a CSV file named `data.csv` in the current directory, and you want to convert it to a JSON file named data.json, you can run the following command:

```bash
$ ./csv-to-json data.csv data.json
```

The tool will read the data from the CSV file, convert it into JSON format, and write the result to the output file.

## CSV Format

The tool expects the input CSV file to have the following format:

```csv
name,url,username,password
accounts.google.com,https://accounts.google.com,myemail@example.xyz,******
accounts.google.com,https://accounts.google.com,myotheremail@example.xyz,******
```

Here, the first line of the file is the header, and the subsequent lines are the data rows. Each data row represents a single record with four fields: `name`, `url`, `username`, and `password`.

## JSON Format

The tool outputs a JSON file with the following format:

```json
{
  "count": 2,
  "results": [
    {
      "id": 1,
      "site": "accounts.google.com",
      "login": "myemail@example.xyz",
      "lowercase": true,
      "uppercase": true,
      "symbols": true,
      "numbers": true,
      "length": 16,
      "counter": 1,
      "version": 2,
      "created": "2023-02-28T13:29:46Z",
      "modified": "2023-02-28T13:29:46Z"
    },
    {
      "id": 2,
      "site": "accounts.google.com",
      "login": "myotheremail@example.xyz",
      "lowercase": true,
      "uppercase": true,
      "symbols": true,
      "numbers": true,
      "length": 16,
      "counter": 1,
      "version": 2,
      "created": "2023-02-28T13:29:46Z",
      "modified": "2023-02-28T13:29:46Z"
    }
  ]
}
```

Here, the `count` field indicates the total number of records in the input CSV file, and the `results` field is an array of objects representing the individual records. Each record has the following fields:

- `id`: a unique identifier for the record, starting from 1 and incrementing by 1 for each subsequent record.
- `site`: the value of the `name` field from the input CSV file.

* `login`: the value of the username field from the input CSV file.
* `lowercase`, `uppercase`, `symbols`, and `numbers`: boolean flags indicating whether the generated password should contain lowercase letters, uppercase letters, symbols, and numbers, respectively. These fields are hard-coded to true in the current implementation.
* `length`: the length of the generated password, which is hard-coded to `16` in the current implementation.
* `counter`: a counter that is incremented each time a password is generated. This field is hard-coded to 1 in the current implementation.
* `version`: a version number for the password generation algorithm. This field is hard-coded to 2 in the current implementation.
* `created` and `modified`: timestamps indicating when the record was created and last modified, respectively. These fields are hard-coded to the current date and time in UTC in the format `YYYY-MM-DDTHH:MM:SSZ`.

## Contributing

Contributions to this tool are welcome. If you find a bug or have an idea for an improvement, please open an issue or submit a pull request on GitHub.

## License

This tool is licensed under the MIT License. See the LICENSE file for details.
