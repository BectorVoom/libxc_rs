//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1150/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1150<F: Float>(t2379: F, t28: F, t2752: F, t13487: F, t1081: F, t776: F, t2553: F, t2749: F, t868: F, t2745: F, t1877: F, t1915: F, t22959: F, t23286: F, t23290: F, t23295: F, t2522: F, t3231: F, t4314: F, t6666: F, t6670: F, t6841: F, t6848: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23781 = t28 * t2379;
    let t23788 = t2752 * t28;
    let t23789 = t23788 * t13487;
    let t23792 = t1081 * t776;
    let t23796 = t28 * t2553;
    let t23807 = t28 * t2749;
    let t23810 = t1081 * t868;
    let t23813 = t28 * t2745;
    let t23820 = F::new(3.0) * t4314 * t1915 * t23781 + F::new(3.0) * t2522 * t6666 * t6841 - F::new(3.0) * t22959 * t23789 + F::new(3.0) * t2522 * t1915 * t23792 + F::new(3.0) / F::new(2.0) * t2522 * t1915 * t23796 + t1877 * t23286 * t28 / F::new(2.0) - t1877 * t23290 * t6848 + t1877 * t6666 * t1081 + t1877 * t23295 * t23807 - t1877 * t6670 * t23810 - t1877 * t6670 * t23813 / F::new(2.0) + t1877 * t1915 * t3231 / F::new(2.0);
    (t23781, t23788, t23789, t23792, t23796, t23807, t23810, t23813, t23820)
}
