import argparse
import fib_rs

def fib_num_cli() -> None:
    parser = argparse.ArgumentParser(description="Calculate Fibonacci numbers")
    parser.add_argument(
        "--number",
        action="store",
        type=int,
        required=True,
        help="Fibonacci number to be calculated",
    )
    args = parser.parse_args()
    print(f"Your fibonacci number is: {fib_rs.fibonacci_number(n=args.number)}")
