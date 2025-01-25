#Temperature Converter

This simple command-line program allows users to convert temperatures between Celsius and Fahrenheit. It provides an interactive interface where the user can input the desired temperature scale (C for Celsius or F for Fahrenheit), input a temperature value, and receive the converted result.

The program continues to run in a loop, allowing multiple conversions until the user opts to quit.
##Features

    Converts temperatures between Celsius and Fahrenheit.
    Handles invalid input gracefully with clear prompts.
    Continuous interaction until the user chooses to quit.

##Usage

    Start the Program: Upon running the program, you will be prompted to input the temperature scale you'd like to convert from:
        Input c to convert from Celsius to Fahrenheit.
        Input f to convert from Fahrenheit to Celsius.
        Input q to quit the program.

    Input a Temperature: After selecting a scale, you will be asked to input the temperature you'd like to convert. The program will then display the converted temperature.

##Example

Please input the temperature scale you want to convert
Input c for Celsius or f for Fahrenheit
Input q to quit the program
Please input temperature scale (c/f): c
Please input the temperature you want to convert: 25
The converted value is 77.00 Fahrenheit

Please input the temperature scale you want to convert
Input c for Celsius or f for Fahrenheit
Input q to quit the program
Please input temperature scale (c/f): f
Please input the temperature you want to convert: 77
The converted value is 25.00 Celsius

##Code Explanation

    The program uses a loop to continuously prompt the user for input until q is entered.
    The user selects a temperature scale (c or f), followed by entering a temperature value.
    The program performs the conversion using the formulas:
        Celsius to Fahrenheit: (cel * 1.8) + 32
        Fahrenheit to Celsius: (fah - 32) / 1.8
    Invalid input is handled with error messages and prompts the user to re-enter the values.

##Requirements

    Rust 1.0 or higher

##How to Run

    Clone this repository:

git clone https://github.com/yourusername/temperature-converter.git

Navigate to the project directory:

cd temperature-converter

Run the program:

    cargo run

