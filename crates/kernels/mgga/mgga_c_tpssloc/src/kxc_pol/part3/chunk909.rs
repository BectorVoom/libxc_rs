//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 909/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk909<F: Float>(t40: F, t12932: F, t707: F, t2379: F, t262: F, t157: F, t9897: F, t2244: F, t4195: F, t2371: F, t4199: F, t1409: F, t2517: F, t3966: F, t75: F, t12606: F, t1430: F, t2250: F, t4104: F, t607: F, t767: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t12934 = 8.0 * t707 * t12932;
    let t12935 = t2379 * t262;
    let t12939 = t9897 * t157;
    let t12940 = t4195 * t2244;
    let t12942 = 24.0 * t12939 * t12940;
    let t12943 = t4199 * t2371;
    let t12944 = 0.11696447245269292414e1 * t12943;
    let t12945 = t2517 * t1409;
    let t12946 = t707 * t12945;
    let t12947 = 4.0 * t12946;
    let t12950 = t75 * t3966;
    let t12958 = piecewise3(t146, 0.0, 8.0 / 27.0 * t1430 * t2244 - 4.0 / 9.0 * t12950 * t607 - 2.0 / 9.0 * t4104 * t2250 + 2.0 / 3.0 * t767 * t12606);
    (t12934, t12935, t12942, t12944, t12947, t12958)
}
