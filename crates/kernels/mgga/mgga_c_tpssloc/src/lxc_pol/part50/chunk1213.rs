//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1213/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1213<F: Float>(t23384: F, t32973: F, t1052: F, t113201: F, t113219: F, t113261: F, t113601: F, t113608: F, t113611: F, t1599: F, t1635: F, t1945: F, t23346: F, t23372: F, t25452: F, t25712: F, t25742: F, t25757: F, t25758: F, t30778: F, t30793: F, t30915: F, t3174: F, t32981: F, t33001: F, t343: F, t4557: F, t4660: F, t4665: F, t6687: F, t6690: F, t6815: F, t7553: F, t7624: F, t7625: F) -> F {
    let t119559 = t23384 * t32973;
    let t119571 = F::cast_from(0.14621636149762012769e-1_f64) * t113601 + F::new(4.0) * t1052 * t3174 * t6815 * t7624 + F::cast_from(0.54831135561607547883e-2_f64) * t6687 * t113261 * t7553 - F::new(2.0) * t23372 * t7625 + F::new(4.0) * t4660 * t30793 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t1599 * t113201 + F::new(2.0) * t4557 * t30778 - F::cast_from(0.87729816898572076613e-1_f64) * t23346 * t32981 - F::cast_from(0.54831135561607547883e-2_f64) * t113608 + F::cast_from(0.43864908449286038307e-1_f64) * t23346 * t33001 + F::cast_from(0.54831135561607547883e-2_f64) * t113611 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t25712 * t343 * t1945 * t6690 - t113219 * t1635 + F::cast_from(0.54831135561607547883e-2_f64) * t119559 - F::cast_from(0.43864908449286038307e-1_f64) * t23346 * t32973 + F::new(2.0) * t30915 * t4665 - F::new(12.0) * t25757 * t25758 * t25452 - F::new(12.0) * t25757 * t25758 * t25742;
    t119571
}
