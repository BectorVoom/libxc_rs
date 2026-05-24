//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1143/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1143<F: Float>(t12749: F, t9957: F, t12738: F, t12739: F, t12740: F, t12741: F, t12742: F, t12745: F, t12747: F, t12748: F, t7945: F, t7954: F, t7960: F, t7972: F, t7975: F, t9886: F, t9903: F, t9906: F, t9954: F, t9956: F) -> (F, F, F) {
    let t12750 = F::new(20.0) * t12749;
    let t12751 = F::cast_from(0.4883052614935078681e-3_f64) * t9957;
    let t12752 = t7945 - t12738 + t9886 - t12739 + t12740 + t12741 + t9903 - t9906 - t7954 - t12742 + t12745 - t7960 + t7972 + t7975 + t12747 - t12748 + t12750 - t9954 + t9956 + t12751;
    (t12750, t12751, t12752)
}
