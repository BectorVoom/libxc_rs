//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 720/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk720<F: Float>(t6579: F, t6649: F, t22997: F, t232: F, t6646: F, t1888: F, t1902: F, t2627: F, t2633: F, t1879: F, t22715: F) -> (F, F, F, F, F) {
    let t23002 = t6579 * t6649;
    let t23003 = F::new(0.38381794893125283518e-1) * t23002;
    let t23004 = t22997 * t232;
    let t23005 = t6646 * t23004;
    let t23006 = t1888 * t23005;
    let t23008 = t2627 * t1902;
    let t23009 = t23008 * t2633;
    let t23012 = t22715 * t1879;
    (t23002, t23003, t23006, t23009, t23012)
}
