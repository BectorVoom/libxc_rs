//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2609/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2609<F: Float>(t11791: F, t5024: F, t11820: F, t5002: F, t11153: F, t4899: F, t3540: F, t4961: F, t11709: F, t15640: F, t11738: F, t15535: F, t15553: F, t3447: F, t3516: F, t44965: F, t44968: F, t44972: F, t44976: F, t44982: F, t4582: F, t45971: F) -> F {
    let t52991 = t5024 * t11791;
    let t52992 = t52991 / F::new(1296.0);
    let t52993 = t5002 * t11820;
    let t52994 = t52993 / F::new(4608.0);
    let t52995 = t4899 * t11153;
    let t52999 = t4961 * t3540;
    let t53000 = t52999 / F::new(864.0);
    let t53001 = t11709 * t15640;
    let t53013 = -t52992 - t52994 + t3447 * t52995 * t45971 / F::new(12.0) + t53000 + t53001 / F::new(384.0) + t44965 * t15535 / F::new(1024.0) + t11738 * t4582 * t15553 * t3516 / F::new(1024.0) + t44968 / F::new(3456.0) + t44972 / F::new(6912.0) + t44976 / F::new(3456.0) - t44982 / F::new(1152.0);
    t53013
}
