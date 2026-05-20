//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2107/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2107<F: Float>(t4166: F, t9666: F, t2693: F, t4163: F, t41008: F, t4155: F, t41115: F, t4240: F, t1512: F, t41340: F, t4236: F, t9671: F) -> (F, F, F, F, F, F) {
    let t46881 = t4166 * t9666;
    let t46886 = t4163 * t2693;
    let t46887 = F::new(119.0) / F::new(4608.0) * t46886;
    let t46911 = t41008 * t4155;
    let t46912 = F::new(35.0) / F::new(24.0) * t46911;
    let t46928 = t41115 * t4240;
    let t46929 = F::new(119.0) / F::new(4608.0) * t46928;
    let t46951 = t41340 * t1512;
    let t46952 = F::new(119.0) / F::new(4608.0) * t46951;
    let t46953 = t9671 * t4236;
    (t46881, t46887, t46912, t46929, t46952, t46953)
}
