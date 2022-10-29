#[cfg(test)]
mod tests {
    use crate::models::{
        mapper::{Exchange, OfferData},
        messages::{OrderbookMessage, Orders},
        stream_service::StreamService,
    };
    use approx::assert_relative_eq;

    /// Tests that we sort and convert the list of bids accordingly
    #[tokio::test]
    async fn test_sort_and_convert() {
        let mut asks = vec![
            OfferData {
                price: 50.0,
                quantity: 0.8,
            },
            OfferData {
                price: 30.0,
                quantity: 5.5,
            },
            OfferData {
                price: 100.0,
                quantity: 2.6,
            },
        ];

        let mut bids = vec![
            OfferData {
                price: 20.0,
                quantity: 1.2,
            },
            OfferData {
                price: 70.0,
                quantity: 7.1,
            },
            OfferData {
                price: 65.0,
                quantity: 1.5,
            },
        ];

        let (converted_asks, converted_bids) =
            StreamService::sort_and_convert(&mut asks, &mut bids, &Exchange::Binance);

        assert_eq!(converted_asks.len(), 3);
        assert_eq!(converted_bids.len(), 3);

        assert_relative_eq!(converted_asks[0].price, 30.0);
        assert_relative_eq!(converted_asks[0].amount, 5.5);
        assert_relative_eq!(converted_asks[1].price, 50.0);
        assert_relative_eq!(converted_asks[1].amount, 0.8, max_relative = 0.000001);
        assert_relative_eq!(converted_asks[2].price, 100.0);
        assert_relative_eq!(converted_asks[2].amount, 2.6, max_relative = 0.000001);

        assert_relative_eq!(converted_bids[0].price, 70.0);
        assert_relative_eq!(converted_bids[0].amount, 7.1, max_relative = 0.000001);
        assert_relative_eq!(converted_bids[1].price, 65.0);
        assert_relative_eq!(converted_bids[1].amount, 1.5, max_relative = 0.000001);
        assert_relative_eq!(converted_bids[2].price, 20.0);
        assert_relative_eq!(converted_bids[2].amount, 1.2, max_relative = 0.000001);
    }

    /// Tests that we get only 10 asks and 10 bids to return
    #[tokio::test]
    async fn test_sort_and_convert_max_ten() {
        let mut asks = vec![
            OfferData {
                price: 50.0,
                quantity: 0.8
            };
            100
        ];

        let mut bids = vec![
            OfferData {
                price: 20.0,
                quantity: 1.2
            };
            100
        ];

        assert_eq!(asks.len(), 100);
        assert_eq!(bids.len(), 100);

        let (converted_asks, converted_bids) =
            StreamService::sort_and_convert(&mut asks, &mut bids, &Exchange::Binance);

        assert_eq!(converted_asks.len(), 10);
        assert_eq!(converted_bids.len(), 10);
    }

    /// Tests handle message and that we calculate spread accordingly
    #[tokio::test]
    async fn test_handle_message() {
        let msg = OrderbookMessage::Message {
            message: Box::new(Orders {
                asks: vec![
                    OfferData {
                        price: 75.0,
                        quantity: 0.8,
                    },
                    OfferData {
                        price: 60.0,
                        quantity: 5.5,
                    },
                    OfferData {
                        price: 65.0,
                        quantity: 2.1,
                    },
                ],
                bids: vec![
                    OfferData {
                        price: 49.0,
                        quantity: 7.1,
                    },
                    OfferData {
                        price: 53.0,
                        quantity: 1.5,
                    },
                    OfferData {
                        price: 51.0,
                        quantity: 7.2,
                    },
                ],
                exchange: Exchange::Binance,
            }),
        };

        let summary = StreamService::handle_message(&msg).expect("ok");

        // The spread should be (60.0 - 53.0) == 7.0. That's because the best ask price is 60.0
        // and the best big price is 53.0
        assert_relative_eq!(summary.spread, 7.0);
        assert_eq!(summary.asks.len(), 3);
        assert_eq!(summary.bids.len(), 3);

        // Test that asks and bids are properly ordered
        assert_relative_eq!(summary.asks[0].price, 60.0);
        assert_relative_eq!(summary.asks[0].amount, 5.5);
        assert_relative_eq!(summary.asks[1].price, 65.0);
        assert_relative_eq!(summary.asks[1].amount, 2.1, max_relative = 0.000001);
        assert_relative_eq!(summary.asks[2].price, 75.0);
        assert_relative_eq!(summary.asks[2].amount, 0.8, max_relative = 0.000001);

        assert_relative_eq!(summary.bids[0].price, 53.0);
        assert_relative_eq!(summary.bids[0].amount, 1.5, max_relative = 0.000001);
        assert_relative_eq!(summary.bids[1].price, 51.0);
        assert_relative_eq!(summary.bids[1].amount, 7.2, max_relative = 0.000001);
        assert_relative_eq!(summary.bids[2].price, 49.0);
        assert_relative_eq!(summary.bids[2].amount, 7.1, max_relative = 0.000001);
    }
}
