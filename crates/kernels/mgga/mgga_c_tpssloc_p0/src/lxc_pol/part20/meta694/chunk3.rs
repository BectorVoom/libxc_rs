//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2646/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2646<F: Float>(t1831: F, t40059: F, t16336: F, t3872: F, t12336: F, t12361: F, t1363: F, t1367: F, t16321: F, t16333: F, t3783: F, t40287: F, t5240: F, t5314: F, t53856: F, t53882: F, t53883: F, t53893: F, t53895: F, t53897: F, t820: F) -> F {
    let t53901 = t40059 * t1831;
    let t53903 = t16336 * t3872;
    let t53905 = F::new(5.0) / F::new(256.0) * t16321 * t3872 - t5240 * t12361 / F::new(768.0) - t53882 + F::new(7.0) / F::new(384.0) * t53883 - t1363 * t1367 * t820 * t53856 / F::new(768.0) - t12336 * t5314 / F::new(256.0) - t3783 * t16333 / F::new(256.0) + F::new(7.0) / F::new(384.0) * t53893 + F::new(7.0) / F::new(384.0) * t53895 + F::new(7.0) / F::new(192.0) * t53897 - t40287 * t1831 / F::new(768.0) + F::new(595.0) / F::new(2592.0) * t53901 - F::new(35.0) / F::new(384.0) * t53903;
    t53905
}
