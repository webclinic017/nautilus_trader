# -------------------------------------------------------------------------------------------------
#  Copyright (C) 2015-2020 Nautech Systems Pty Ltd. All rights reserved.
#  The use of this source code is governed by the license as found in the LICENSE file.
#  https://nautechsystems.io
# -------------------------------------------------------------------------------------------------

import unittest

from nautilus_trader.core.decimal import Decimal
from nautilus_trader.model.identifiers import Symbol, Venue
from nautilus_trader.backtest.loaders import InstrumentLoader


class BacktestLoadersTests(unittest.TestCase):

    def test_default_fx_with_5_dp_returns_expected_instrument(self):
        # Arrange
        loader = InstrumentLoader()

        # Act
        instrument = loader.default_fx_ccy(Symbol('AUDUSD', Venue('DUKASCOPY')))

        # Assert
        self.assertEqual(Symbol('AUDUSD', Venue('DUKASCOPY')), instrument.symbol)
        self.assertEqual('AUD/USD', instrument.broker_symbol)
        self.assertEqual(5, instrument.price_precision)
        self.assertEqual(Decimal(0.00001, 5), instrument.tick_size)
        self.assertEqual(840, instrument.quote_currency)

    def test_default_fx_with_3_dp_returns_expected_instrument(self):
        # Arrange
        loader = InstrumentLoader()

        # Act
        instrument = loader.default_fx_ccy(Symbol('USDJPY', Venue('DUKASCOPY')))

        # Assert
        self.assertEqual(Symbol('USDJPY', Venue('DUKASCOPY')), instrument.symbol)
        self.assertEqual('USD/JPY', instrument.broker_symbol)
        self.assertEqual(3, instrument.price_precision)
        self.assertEqual(Decimal(0.001, 3), instrument.tick_size)
        self.assertEqual(392, instrument.quote_currency)