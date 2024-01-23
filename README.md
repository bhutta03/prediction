# Prediction Smart Contract

## Overview

This repository contains the Rust code for a prediction smart contract built on the Solana blockchain using the Anchor framework. The smart contract allows users to participate in a prediction by initializing the, entering up or down positions, and closing positions. The outcome of closed positions is determined by comparing the closed price to the locked price.

## Features

- **Initialize:** Initialize the prediction with a specified locked price.
- **Enter Up Position:** Users can enter an up position, incrementing the locked price.
- **Enter Down Position:** Users can enter a down position, decrementing the locked price.
- **Close Position:** Close a position by providing a closed price. The contract determines if the position is a win, loss, or tie based on the comparison with the locked price.

## Usage



1. Clone the repository:

   ```bash
   git clone https://github.com/bhutta03/prediction_market.git
   cd prediction_market
