//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1429/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1429<F: Float>(t12923: F, t607: F, t4194: F, t3966: F, t751: F, t707: F, t157: F, t9897: F, t2371: F, t4199: F, t1409: F, t2517: F) -> (F, F, F, F, F, F, F) {
    let t12924 = t12923 * t607;
    let t12926 = F::cast_from(24.0_f64) * t4194 * t12924;
    let t12932 = t751 * t3966;
    let t12934 = F::cast_from(8.0_f64) * t707 * t12932;
    let t12939 = t9897 * t157;
    let t12943 = t4199 * t2371;
    let t12945 = t2517 * t1409;
    (t12924, t12926, t12932, t12934, t12939, t12943, t12945)
}
