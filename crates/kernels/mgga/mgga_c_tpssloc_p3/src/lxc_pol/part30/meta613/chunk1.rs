//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2011/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2011<F: Float>(t23471: F, t23482: F, t10889: F, t23535: F, t3033: F, t1016: F, t3034: F, t1930: F, t23418: F, t3180: F, t10401: F, t23417: F) -> (F, F, F, F, F) {
    let t82943 = t23482 * t23471;
    let t82956 = t3033 * t23535 * t10889;
    let t82985 = F::new(1.0) / t3034 / t1016;
    let t82986 = t1930 * t82985;
    let t83008 = t3180 * t23418;
    let t83015 = t23417 * t10401;
    (t82943, t82956, t82986, t83008, t83015)
}
