//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1459/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1459<F: Float>(t11285: F, t11350: F, t11352: F, t1148: F, t1156: F, t1683: F, t1695: F, t18840: F, t18899: F, t21855: F, t21887: F, t21890: F, t21939: F, t21942: F, t3359: F, t43692: F, t44155: F, t44223: F, t44361: F, t4797: F, t4835: F, t51376: F, t51427: F, t51604: F, t6037: F, t6053: F, t6056: F, t6085: F, t6088: F, t63602: F, t64103: F, t64292: F, t71860: F, t71863: F, t78114: F, t78287: F, t78859: F) -> F {
    let t78914 = -F::new(12.0) * t64292 * t6037 - F::cast_from(0.77193501593724168322e3_f64) * t51427 * t21855 + F::cast_from(0.11579025239058625248e4_f64) * t11350 * t78859 * t3359 + F::cast_from(0.23392894490538584828e1_f64) * t71860 * t1695 + F::cast_from(0.35089341735807877242e1_f64) * t18899 * t6085 + F::cast_from(0.10389515463408878255e3_f64) * t63602 * t6088 + F::cast_from(0.23392894490538584828e1_f64) * t4835 * t21939 + F::cast_from(0.4101607543286562663e4_f64) * t51376 * t21942 - F::cast_from(0.12304822629859687989e5_f64) * t44155 * t78287 * t11285 + F::cast_from(0.5848223622634646207e0_f64) * t1148 * t78114 * t1156 + F::cast_from(0.91082604192152556044e5_f64) * t44223 * t78287 * t43692 + F::new(4.0) * t71863 * t1683 + F::new(6.0) * t18840 * t6053 + F::cast_from(0.1929837539843104208e3_f64) * t64103 * t6056 + F::new(4.0) * t4797 * t21887 + F::cast_from(0.82761620670837440481e4_f64) * t51604 * t21890 - F::cast_from(0.24828486201251232145e5_f64) * t44361 * t78859 * t11352;
    t78914
}
