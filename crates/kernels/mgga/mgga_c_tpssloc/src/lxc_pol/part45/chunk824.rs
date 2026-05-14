//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 824/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk824<F: Float>(t10143: F, t1081: F, t28: F, t40772: F, t6514: F, t671: F, t1868: F, t2363: F, t2018: F, t3734: F, t1307: F, t1385: F, t1373: F, t254: F, t1324: F, t6875: F, t8944: F) -> (F, F, F, F, F, F, F, F, F) {
    let t89849 = t10143 * t1081;
    let t89953 = t40772 * t28;
    let t90041 = t6514 * t671;
    let t90044 = t1868 * t2363;
    let t90065 = t3734 * t2018;
    let t90506 = t1307 * t1385;
    let t90665 = t1373 * t254;
    let t91505 = t1324 * t254;
    let t91669 = t6875 * t8944;
    (t89849, t89953, t90041, t90044, t90065, t90506, t90665, t91505, t91669)
}
