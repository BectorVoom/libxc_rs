//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 970/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk970<F: Float>(t10022: F, t248: F, t557: F, t555: F, t10027: F, t541: F, t12267: F, t1362: F, t3777: F, t3865: F, t1369: F, t1361: F, t2690: F) -> (F, F, F, F, F, F) {
    let t12328 = t10022 * t557 * t248;
    let t12330 = F::new(595.0) / F::new(10368.0) * t555 * t12328;
    let t12335 = F::new(455.0) / F::new(1296.0) * t10027 * t541;
    let t12336 = t12267 * t1362;
    let t12339 = t3777 * t3865;
    let t12340 = t12339 * t1369;
    let t12344 = t1361 * t2690;
    (t12330, t12335, t12336, t12339, t12340, t12344)
}
