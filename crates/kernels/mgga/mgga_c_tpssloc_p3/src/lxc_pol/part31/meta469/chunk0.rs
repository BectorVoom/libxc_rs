//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1628/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1628<F: Float>(t4072: F, t89: F, t22751: F, t7692: F, t22666: F, t7691: F, t6888: F, t5187: F, t6890: F, t6889: F, t1834: F, t214: F) -> (F, F, F, F, F, F, F, F) {
    let t26179 = t89 * t4072;
    let t26184 = t22751 * t7692;
    let t26186 = t22666 * t7691;
    let t26187 = t6888 * t26186;
    let t26189 = t6890 * t5187;
    let t26190 = t6889 * t26189;
    let t26191 = t6888 * t26190;
    let t26193 = t214 * t1834;
    (t26179, t26184, t26186, t26187, t26189, t26190, t26191, t26193)
}
