//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1208/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1208<F: Float>(t32840: F, t8344: F, t232: F, t4180: F, t4181: F, t30714: F, t1516: F, t8343: F, t30698: F, t30705: F, t30722: F, t32835: F, t32838: F) -> (F, F) {
    let t32841 = t32840 * t8344;
    let t32844 = t4180 * t4181 * t232;
    let t32845 = t30714 * t32844;
    let t32847 = t8343 * t1516;
    let t32849 = -t30698 - F::new(0.48447307312968469025e-2) * t32835 - t30705 - F::new(0.80745512188280781708e-3) * t32838 + t32841 / F::new(1536.0) - t32845 / F::new(1536.0) - t30722 - t32847 / F::new(384.0);
    (t32844, t32849)
}
