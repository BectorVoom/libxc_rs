//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta552 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1951;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta552<F: Float>(t27516: F, t7364: F, t5072: F, t7376: F, t7375: F, t1215: F, t1409: F, t24851: F, t24589: F, t24812: F, t24827: F, t24849: F, t27406: F, t27481: F, t27484: F, t27492: F, t27498: F, t27502: F, t27507: F, t27511: F, t7283: F, t7368: F, t7373: F, t7378: F) -> (F, F, F, F, F, F) {
        let (t27517, t27520, t27521, t27525, t27526, t27529) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1951::<F>(t27516, t7364, t5072, t7376, t7375, t1215, t1409, t24851, t24589, t24812, t24827, t24849, t27406, t27481, t27484, t27492, t27498, t27502, t27507, t27511, t7283, t7368, t7373, t7378);
    (t27517, t27520, t27521, t27525, t27526, t27529)
}
