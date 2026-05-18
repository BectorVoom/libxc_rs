//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1332/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1332<F: Float>(t1395: F, t8256: F, t1404: F, t8240: F, t2186: F, t5381: F, t30217: F, t580: F, t110014: F, t110018: F, t110489: F, t110655: F, t110872: F, t110877: F, t1398: F, t16507: F, t1852: F, t1858: F, t2193: F, t29979: F, t3: F, t30017: F, t3932: F) -> F {
    let t110882 = F::new(2.0) * t1395 * t8256;
    let t110884 = F::new(2.0) * t8240 * t1404;
    let t110886 = F::new(2.0) * t2186 * t5381;
    let t110888 = F::new(2.0) * t30217 * t580;
    let t110893 = t110489 + t1852 * t30017 + t29979 * t1858 + t1398 * (t110655 + t110877) + t16507 * t2193 + t110882 + t110884 + t110886 + t110888 + t3 * t110872 * t580 + t110018 + t3932 * t8256 + F::new(2.0) * t110014;
    t110893
}
