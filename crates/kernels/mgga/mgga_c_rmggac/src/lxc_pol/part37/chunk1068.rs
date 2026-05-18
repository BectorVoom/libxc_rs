//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1068/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1068<F: Float>(t71219: F, t74969: F, t74975: F, t74979: F, t74981: F, t74983: F, t74986: F, t77323: F, t77329: F, t77332: F, t77337: F, t77340: F, t77343: F, t77347: F, t77349: F, t77352: F, t77353: F) -> F {
    let t80191 = -t71219 + t77323 - F::new(0.46594213659335792121e-1) * t74969 + F::new(0.93188427318671584242e-1) * t74975 + F::new(0.15531404553111930707e-1) * t74979 - t77329 + t77332 + t77337 - t77340 + t77343 + t77347 - t74981 + t77349 - t74983 + t77352 + t74986 - t77353;
    t80191
}
