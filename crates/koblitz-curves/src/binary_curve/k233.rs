/// Instantiation of Binary Curve with K-233, E(\bar{K}): x^2 + xy = x^3 + 1, where \bar{K} = GF(2)[X] / X^233 + X^74 + 1
use super::curve::{BinaryCurve, ProjectivePoint};
use crate::binary_field::{N, fq233::Fq233, polynomial::BinaryPolynomial};
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct K233;

impl BinaryCurve<N, Fq233> for K233 {
    const A2: Fq233 = Fq233(BinaryPolynomial([0, 0, 0, 0, 0, 0, 0, 0]));
    const A6: Fq233 = Fq233(BinaryPolynomial([1, 0, 0, 0, 0, 0, 0, 0]));
    const IDENTITY: ProjectivePoint<N, Fq233, Self> = ProjectivePoint {
        x: Fq233(BinaryPolynomial([1, 0, 0, 0, 0, 0, 0, 0])),
        y: Fq233(BinaryPolynomial([0, 0, 0, 0, 0, 0, 0, 0])),
        z: Fq233(BinaryPolynomial([0, 0, 0, 0, 0, 0, 0, 0])),
        marker: PhantomData,
    };
    const GENERATOR: ProjectivePoint<N, Fq233, Self> = ProjectivePoint {
        x: Fq233(BinaryPolynomial([
            1725572810, 2000923818, 1177071851, 1254338503, 3682277809, 1791495317, 2034630957, 251,
        ])),
        y: Fq233(BinaryPolynomial([
            691743371, 1573401319, 2498410245, 2411648772, 520441339, 3254992135, 3793809428, 211,
        ])),
        z: Fq233(BinaryPolynomial([1, 0, 0, 0, 0, 0, 0, 0])),
        marker: PhantomData,
    };
    const A6_SQRT: Fq233 = Fq233(BinaryPolynomial([1, 0, 0, 0, 0, 0, 0, 0]));
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::binary_curve::curve::ProjectivePoint;

    #[test]
    fn test_is_on_curve() {
        let g = K233::GENERATOR;
        assert!(g.is_on_curve(), "Test for K233 is_on_curve failed!");
    }

    #[test]
    fn test_addition() {
        let test_data = [(
            (
                String::from_str(
                    "0x000000fb7946012d6ac80c95db7b19b14ac3afc74628b0eb7743acaa66da26ca",
                )
                .unwrap(),
                String::from_str(
                    "0x000000d3e220f014c2033d071f054dfb8fbed70494eab7055dc832e7293b2a8b",
                )
                .unwrap(),
                String::from_str(
                    "0x0000000000000000000000000000000000000000000000000000000000000001",
                )
                .unwrap(),
            ),
            (
                String::from_str(
                    "0x000000a27e23fca9a5c8c45f266277022015dc908bdc4796b9dc03b531949b9c",
                )
                .unwrap(),
                String::from_str(
                    "0x000000678d8d5bd28b8766a778d26db4cd501a95feabce1af002e3979d88a3df",
                )
                .unwrap(),
                String::from_str(
                    "0x0000000000000000000000000000000000000000000000000000000000000001",
                )
                .unwrap(),
            ),
            (
                String::from_str(
                    "0x0000018dd170c7fc91443bee679cc20b0ca53342abc20fb184fe8b6a25701fa5",
                )
                .unwrap(),
                String::from_str(
                    "0x000000917e9e565076614ee7255f38650c3410cade1cad62c22a367700212d4b",
                )
                .unwrap(),
                String::from_str(
                    "0x0000000000000000000000000000000000000000000000000000000000000001",
                )
                .unwrap(),
            ),
        )];

        for (u_hex_string, v_hex_string, w_expected_hex_string) in test_data {
            let (u, v, w_expected) = (
                ProjectivePoint {
                    x: Fq233::from_hex_string(&u_hex_string.0),
                    y: Fq233::from_hex_string(&u_hex_string.1),
                    z: Fq233::from_hex_string(&u_hex_string.2),
                    marker: PhantomData::<K233>,
                },
                ProjectivePoint {
                    x: Fq233::from_hex_string(&v_hex_string.0),
                    y: Fq233::from_hex_string(&v_hex_string.1),
                    z: Fq233::from_hex_string(&v_hex_string.2),
                    marker: PhantomData::<K233>,
                },
                ProjectivePoint {
                    x: Fq233::from_hex_string(&w_expected_hex_string.0),
                    y: Fq233::from_hex_string(&w_expected_hex_string.1),
                    z: Fq233::from_hex_string(&w_expected_hex_string.2),
                    marker: PhantomData::<K233>,
                },
            );
            let w = u + v;
            assert_eq!(w, w_expected, "Test for K233 addition failed!");
        }
    }

    #[test]
    fn test_doubing() {
        let test_data = [(
            (
                String::from_str(
                    "0x000000fb7946012d6ac80c95db7b19b14ac3afc74628b0eb7743acaa66da26ca",
                )
                .unwrap(),
                String::from_str(
                    "0x000000d3e220f014c2033d071f054dfb8fbed70494eab7055dc832e7293b2a8b",
                )
                .unwrap(),
                String::from_str(
                    "0x0000000000000000000000000000000000000000000000000000000000000001",
                )
                .unwrap(),
            ),
            (
                String::from_str(
                    "0x000000a27e23fca9a5c8c45f266277022015dc908bdc4796b9dc03b531949b9c",
                )
                .unwrap(),
                String::from_str(
                    "0x000000678d8d5bd28b8766a778d26db4cd501a95feabce1af002e3979d88a3df",
                )
                .unwrap(),
                String::from_str(
                    "0x0000000000000000000000000000000000000000000000000000000000000001",
                )
                .unwrap(),
            ),
        )];
        for (u_hex_string, w_expected_hex_string) in test_data {
            let (u, w_expected) = (
                ProjectivePoint {
                    x: Fq233::from_hex_string(&u_hex_string.0),
                    y: Fq233::from_hex_string(&u_hex_string.1),
                    z: Fq233::from_hex_string(&u_hex_string.2),
                    marker: PhantomData::<K233>,
                },
                ProjectivePoint {
                    x: Fq233::from_hex_string(&w_expected_hex_string.0),
                    y: Fq233::from_hex_string(&w_expected_hex_string.1),
                    z: Fq233::from_hex_string(&w_expected_hex_string.2),
                    marker: PhantomData::<K233>,
                },
            );
            let w = u + u;
            assert_eq!(w, w_expected, "Test for K233 doubling failed!");
        }
    }

    #[test]
    fn test_montgomery_scalar_mul() {
        let test_data = [
            (
                (
                    String::from_str(
                        "0x000000fb7946012d6ac80c95db7b19b14ac3afc74628b0eb7743acaa66da26ca",
                    )
                    .unwrap(),
                    String::from_str(
                        "0x000000d3e220f014c2033d071f054dfb8fbed70494eab7055dc832e7293b2a8b",
                    )
                    .unwrap(),
                    String::from_str(
                        "0x0000000000000000000000000000000000000000000000000000000000000001",
                    )
                    .unwrap(),
                ),
                String::from_str(
                    "0x0000000000000000000000000000000000000000000000000000000000000003",
                )
                .unwrap(),
                (
                    String::from_str(
                        "0x0000018dd170c7fc91443bee679cc20b0ca53342abc20fb184fe8b6a25701fa5",
                    )
                    .unwrap(),
                    String::from_str(
                        "0x000000917e9e565076614ee7255f38650c3410cade1cad62c22a367700212d4b",
                    )
                    .unwrap(),
                    String::from_str(
                        "0x0000000000000000000000000000000000000000000000000000000000000001",
                    )
                    .unwrap(),
                ),
            ),
            (
                (
                    String::from_str(
                        "0x000000fb7946012d6ac80c95db7b19b14ac3afc74628b0eb7743acaa66da26ca",
                    )
                    .unwrap(),
                    String::from_str(
                        "0x000000d3e220f014c2033d071f054dfb8fbed70494eab7055dc832e7293b2a8b",
                    )
                    .unwrap(),
                    String::from_str(
                        "0x0000000000000000000000000000000000000000000000000000000000000001",
                    )
                    .unwrap(),
                ),
                String::from_str(
                    "0x0000000000000000000000000000000000000000000000000000000000000064",
                )
                .unwrap(),
                (
                    String::from_str(
                        "0x0000009e1bf51cc7587404389afdfb96ffaa7c770ca4efe5cbcd7f74dc3e80cb",
                    )
                    .unwrap(),
                    String::from_str(
                        "0x000000688b323a0497b654e11ecdbb22ecd20642ef7f928821d8c9ca21dbaf32",
                    )
                    .unwrap(),
                    String::from_str(
                        "0x0000000000000000000000000000000000000000000000000000000000000001",
                    )
                    .unwrap(),
                ),
            ),
            (
                (
                    String::from_str(
                        "0x000000fb7946012d6ac80c95db7b19b14ac3afc74628b0eb7743acaa66da26ca",
                    )
                    .unwrap(),
                    String::from_str(
                        "0x000000d3e220f014c2033d071f054dfb8fbed70494eab7055dc832e7293b2a8b",
                    )
                    .unwrap(),
                    String::from_str(
                        "0x0000000000000000000000000000000000000000000000000000000000000001",
                    )
                    .unwrap(),
                ),
                String::from_str(
                    "0x0000017c14c59e6253fa1903f05141fd556d02d1aec2c77b038098981ecf8166",
                )
                .unwrap(),
                (
                    String::from_str(
                        "0x000001b28c591e7773e37179530ffa59fb2c531c39bd4f1715596cdbd1892568",
                    )
                    .unwrap(),
                    String::from_str(
                        "0x0000005f46e4100332eea099da75d3435cd77ba6be13c06f559cef4ba0d06fa9",
                    )
                    .unwrap(),
                    String::from_str(
                        "0x0000000000000000000000000000000000000000000000000000000000000001",
                    )
                    .unwrap(),
                ),
            ),
        ];
        for (u_hex_string, v_hex_string, w_expected_hex_string) in test_data {
            let (u, v, w_expected) = (
                ProjectivePoint {
                    x: Fq233::from_hex_string(&u_hex_string.0),
                    y: Fq233::from_hex_string(&u_hex_string.1),
                    z: Fq233::from_hex_string(&u_hex_string.2),
                    marker: PhantomData::<K233>,
                },
                Fq233::from_hex_string(&v_hex_string),
                ProjectivePoint {
                    x: Fq233::from_hex_string(&w_expected_hex_string.0),
                    y: Fq233::from_hex_string(&w_expected_hex_string.1),
                    z: Fq233::from_hex_string(&w_expected_hex_string.2),
                    marker: PhantomData::<K233>,
                },
            );
            // w = [v]u
            let w = u * v;
            assert_eq!(
                w, w_expected,
                "Test for K233 Montgomery scalar multiplication failed!"
            );
        }
    }
}
