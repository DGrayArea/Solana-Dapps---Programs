# Token Vesting

The program allows for the creation of vesting accounts for companies and employees, enabling token distributions based on predefined vesting schedules.

## Features

Create Vesting Account: Set up a vesting account for a company, specifying the company name and associated accounts.
Create Employee Vesting: Establish a vesting schedule for an employee, including start and end times, total amount, and a cliff period.
Claim Tokens: Allows employees to claim their vested tokens after the cliff period, based on the time elapsed and the amount vested.

## Program Functions

- `create_vesting_account`: Initializes a vesting account for a company and initializes a vesting token account to hold the entire token allocation.
- `create_employee_vesting`: Initializes a vesting schedule for an employee adn initializes an employee token account to receive their unlocked allocation.
- `claim_tokens`: Allows an employee to claim all vested tokens that have unlocked.

## Account Structures

- `CreateEmployeeAccount`: Account structure for creating an employee vesting account.
- `CreateVestingAccount`: Account structure for creating a company's vesting account.
- `ClaimTokens`: Account structure for claiming tokens.

## Data Structures

- `EmployeeAccount`: Stores details about an employee's vesting schedule.
- `VestingAccount`: Stores details about a company's vesting account.
