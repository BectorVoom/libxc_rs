//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2299/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2299<F: Float>(t13062: F, t225: F, t13378: F, t10049: F, t10103: F, t10110: F, t10116: F, t13059: F, t13377: F, t1527: F, t218: F, t252: F, t259: F, t2591: F, t2710: F, t2713: F, t2718: F, t2719: F, t4142: F, t4265: F, t4268: F, t4273: F, t4300: F, t4301: F, t46860: F, t47363: F, t798: F, t855: F, t866: F, t9590: F, t9593: F) -> F {
    let t47609 = t13062 * t225;
    let t47618 = t13378 * t225;
    let t47631 = F::new(2.0) * t10103 * t1527 * t2718 * t855 - F::new(18.0) * t10110 * t2719 * t4300 * t855 + F::new(3.0) * t13377 * t259 * t798 + t218 * t259 * t47363 + t252 * t259 * t46860 + F::new(3.0) * t259 * t2591 * t4265 + F::new(3.0) * t259 * t2710 * t4142 + F::new(6.0) * t10049 * t4273 + F::new(6.0) * t10116 * t4268 + F::new(6.0) * t13059 * t2713 - F::new(3.0) * t4301 * t9590 - F::new(6.0) * t4301 * t9593 - F::new(6.0) * t47609 * t866 - F::new(3.0) * t47618 * t866;
    t47631
}
