//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1822;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1823;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1824;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta466<F: Float>(t225: F, t3166: F, t387: F, t345: F, t1922: F, t2966: F, t1920: F, t1049: F, t6703: F, t6706: F, t6710: F, t6769: F, t1955: F, t3206: F, t3174: F, t10160: F, t1052: F, t1066: F, t1956: F, t23346: F, t3169: F, t3176: F, t3207: F, t6687: F, t6695: F, t6771: F, t6816: F, t134: F, t221: F, t1926: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t23353, t23354, t23357, t23359, t23365, t23366, t23369, t23372) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1822::<F>(t225, t3166, t387, t345, t1922, t2966, t1920, t1049, t6703, t6706, t6710, t6769);
        let (t23378, t23381) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1823::<F>(t1955, t3206, t3174, t10160, t1052, t1066, t1920, t1956, t23346, t23354, t23359, t23366, t23369, t23372, t3169, t3176, t3207, t6687, t6695, t6771, t6816);
        let (t23383, t23384) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1824::<F>(t134, t221, t1926);
    (t23353, t23357, t23359, t23365, t23366, t23369, t23372, t23378, t23381, t23383, t23384)
}
