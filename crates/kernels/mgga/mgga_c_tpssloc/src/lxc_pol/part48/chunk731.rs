//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 731/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk731<F: Float>(t2144: F, t3507: F, t3625: F, t1215: F, t7348: F, t1246: F, t1170: F, t7381: F, t2121: F, t210: F, t7371: F, t7284: F, t974: F, t1089: F, t491: F, t7327: F) -> (F, F, F, F, F, F) {
    let t24837 = t2144 * t3507;
    let t24838 = t24837 * t3625;
    let t24840 = t7348 * t1215;
    let t24841 = t24840 * t1246;
    let t24844 = t1170 * t7381;
    let t24845 = t2121 * t24844;
    let t24847 = t7371 * t210;
    let t24848 = t974 * t7284;
    let t24849 = t24847 * t24848;
    let t24850 = t491 * t1089;
    let t24851 = t7327 * t24850;
    (t24837, t24838, t24841, t24845, t24849, t24851)
}
