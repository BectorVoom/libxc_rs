//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 706/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk706<F: Float>(t23353: F, t345: F, t1922: F, t2966: F, t1920: F, t1049: F, t6703: F, t6706: F, t225: F, t6710: F, t6769: F, t1955: F, t3206: F, t3174: F, t10160: F, t1052: F, t1066: F, t1956: F, t23346: F, t3169: F, t3176: F, t3207: F, t6687: F, t6695: F, t6771: F, t6816: F) -> (F,) {
    let t23354 = t345 * t23353;
    let t23357 = t2966 * t1922;
    let t23359 = 0.18277045187202515961e-2 * t1920 * t23357;
    let t23365 = t6703 * t1049;
    let t23366 = t23365 * t6706;
    let t23369 = t6710 * t225;
    let t23372 = t6769 * t225;
    let t23377 = t1955 * t3206;
    let t23378 = t3174 * t23377;
    let t23381 = 0.43864908449286038306e-1 * t23346 * t6695 + 0.82246703342411321825e-2 * t1920 * t23354 - t23359 - 2.0 * t3169 * t6816 - t6771 * t3207 + 2.0 * t6771 * t3176 - 0.16449340668482264365e-1 * t6687 * t23366 - 2.0 * t23369 * t1066 - 2.0 * t23372 * t1066 - 2.0 * t10160 * t1956 + 2.0 * t1052 * t23378;
    (t23381,)
}
