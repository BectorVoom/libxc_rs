//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1220/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1220<F: Float>(t1369: F, t80866: F, t22782: F, t3777: F, t22783: F, t3876: F, t80807: F, t80810: F, t80814: F, t80817: F, t80821: F, t80826: F, t80828: F, t80831: F, t80833: F, t80837: F, t80843: F, t80848: F, t80850: F, t80857: F, t80859: F, t80861: F, t80863: F) -> F {
    let t80867 = t80866 * t1369;
    let t80869 = t3777 * t22782;
    let t80870 = t80869 * t1369;
    let t80872 = t22783 * t3876;
    let t80874 = F::cast_from(0.10093189023535097714e-3_f64) * t80807 + t80810 / F::new(1536.0) + F::cast_from(0.60559134141210586281e-3_f64) * t80814 + t80817 / F::new(64.0) - F::new(7.0) / F::new(96.0) * t80821 - t80826 - F::new(7.0) / F::new(16.0) * t80828 - t80831 / F::new(4.0) + t80833 / F::new(128.0) + F::cast_from(0.3027956707060529314e-3_f64) * t80837 - F::cast_from(0.42391393898847410397e-2_f64) * t80843 - t80848 - t80850 / F::new(128.0) - F::cast_from(0.12111826828242117256e-2_f64) * t80857 - F::new(35.0) / F::new(192.0) * t80859 - F::new(5.0) / F::new(64.0) * t80861 + F::new(5.0) / F::new(128.0) * t80863 - F::new(119.0) / F::new(576.0) * t80867 + F::new(7.0) / F::new(96.0) * t80870 + F::new(7.0) / F::new(192.0) * t80872;
    t80874
}
