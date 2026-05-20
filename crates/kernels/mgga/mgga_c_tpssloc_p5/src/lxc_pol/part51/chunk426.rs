//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 426/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk426<F: Float>(t1926: F, t350: F, t365: F, t335: F, t371: F) -> (F, F, F, F) {
    let t1927 = t1926 * t350;
    let t1929 = t365 * t365;
    let t1930 = F::new(1.0) / t1929;
    let t1932 = F::new(1.0) / t371 / t335;
    (t1927, t1929, t1930, t1932)
}
