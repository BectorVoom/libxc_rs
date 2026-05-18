//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1335/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1335<F: Float>(t213: F, t81968: F, t1894: F, t236: F, t9458: F, t81907: F, t81909: F, t81912: F, t81918: F, t81921: F, t81924: F, t81926: F, t81928: F, t81930: F, t81934: F, t81936: F, t81940: F, t81943: F, t81946: F, t81949: F, t81955: F, t81957: F, t81960: F, t81964: F) -> F {
    let t81969 = t81968 * t213;
    let t81972 = t81969 * t1894 * t236 * t9458;
    let t81974 = F::new(0.12111826828242117256e-2) * t81907 + F::new(0.42391393898847410397e-2) * t81909 - F::new(0.33913115119077928317e-1) * t81912 - F::new(0.20186378047070195427e-3) * t81918 - t81921 + F::new(0.10093189023535097714e-3) * t81924 - F::new(7.0) / F::new(768.0) * t81926 + F::new(119.0) / F::new(2304.0) * t81928 - t81930 / F::new(48.0) - F::new(0.2034786907144675699e0) * t81934 + F::new(0.25434836339308446238e-1) * t81936 - F::new(0.12111826828242117256e-2) * t81940 - F::new(35.0) / F::new(72.0) * t81943 + F::new(3.0) / F::new(16.0) * t81946 + F::new(0.25434836339308446237e-1) * t81949 - t81955 - F::new(7.0) / F::new(16.0) * t81957 - t81960 / F::new(4.0) - F::new(0.17804385437515912366e0) * t81964 - F::new(0.67826230238155856634e-1) * t81972;
    t81974
}
