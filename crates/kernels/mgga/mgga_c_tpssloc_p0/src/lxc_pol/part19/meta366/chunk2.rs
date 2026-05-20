//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1334/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1334<F: Float>(t10190: F, t10255: F, t2986: F, t2989: F, t9258: F, t10337: F, t964: F, t340: F, t625: F, t221: F, t339: F, t344: F) -> (F, F, F, F, F) {
    let t42794 = t2986 * t10190 * t10255;
    let t42799 = t2989 * t9258;
    let t42811 = t964 * t10337;
    let t42813 = t625 * t340;
    let t42817 = F::cast_from(0.82304526748971193413e-3_f64) * t339 * t221 * t42813 * t344;
    (t42794, t42799, t42811, t42813, t42817)
}
