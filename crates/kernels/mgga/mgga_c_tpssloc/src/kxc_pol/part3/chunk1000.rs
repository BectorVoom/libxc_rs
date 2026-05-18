//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1000/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1000<F: Float>(t9864: F, t9866: F, t3966: F, t751: F, t707: F, t2379: F, t262: F, t157: F, t9897: F, t2244: F, t4195: F, t2371: F, t4199: F) -> (F, F, F, F, F, F) {
    let t12927 = F::new(0.23392894490538584828e1) * t9864;
    let t12928 = F::new(0.34631718211362927518e2) * t9866;
    let t12932 = t751 * t3966;
    let t12934 = F::new(8.0) * t707 * t12932;
    let t12935 = t2379 * t262;
    let t12939 = t9897 * t157;
    let t12940 = t4195 * t2244;
    let t12942 = F::new(24.0) * t12939 * t12940;
    let t12943 = t4199 * t2371;
    (t12927, t12928, t12934, t12935, t12942, t12943)
}
