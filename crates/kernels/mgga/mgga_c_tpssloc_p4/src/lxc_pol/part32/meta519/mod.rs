//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta519 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1851;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta519<F: Float>(t1352: F, t26421: F, t6976: F, t22633: F, t22705: F, t7736: F, t22704: F, t6883: F, t7741: F, t1332: F, t2013: F, t22693: F, t22707: F, t26379: F, t26381: F, t26386: F, t26390: F, t26393: F, t26398: F, t26401: F, t26404: F, t26406: F, t26412: F, t26416: F, t26419: F, t5230: F, t5344: F, t544: F, t7747: F) -> (F, F, F, F, F, F) {
        let (t26422, t26423, t26426, t26427, t26429, t26431) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1851::<F>(t1352, t26421, t6976, t22633, t22705, t7736, t22704, t6883, t7741, t1332, t2013, t22693, t22707, t26379, t26381, t26386, t26390, t26393, t26398, t26401, t26404, t26406, t26412, t26416, t26419, t5230, t5344, t544, t7747);
    (t26422, t26423, t26426, t26427, t26429, t26431)
}
