//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 954/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk954<F: Float>(t157: F, t9929: F, t4196: F, t9726: F, t10143: F, t1530: F, t2430: F, t4205: F, t1409: F, t750: F, t607: F, t4194: F, t9864: F, t9866: F, t3966: F, t751: F) -> (F, F, F, F, F, F, F, F) {
    let t12908 = t9929 * t157;
    let t12910 = 24.0 * t12908 * t4196;
    let t12914 = 2.0 * t9726;
    let t12915 = t1530 * t10143;
    let t12922 = 8.0 * t4205 * t2430;
    let t12923 = t750 * t1409;
    let t12924 = t12923 * t607;
    let t12926 = 24.0 * t4194 * t12924;
    let t12927 = 0.23392894490538584828e1 * t9864;
    let t12928 = 0.34631718211362927518e2 * t9866;
    let t12932 = t751 * t3966;
    (t12910, t12914, t12915, t12922, t12926, t12927, t12928, t12932)
}
