//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1045/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1045<F: Float>(t68409: F, t70799: F, t73971: F, t73974: F, t73977: F, t73981: F, t73984: F, t73994: F, t74008: F, t76803: F, t76804: F, t76805: F, t76808: F, t76814: F, t76816: F, t76817: F, t76820: F) -> F {
    let t80022 = -t76803 + t76804 + t76805 + t68409 + t70799 + t76808 + F::new(0.52557918278704101558e-5) * t73971 - F::new(0.52557918278704101558e-5) * t73974 - F::new(0.17519306092901367186e-5) * t73977 + F::new(0.17519306092901367186e-5) * t73981 - F::new(0.17519306092901367186e-5) * t73984 + t76814 - F::new(0.17451485956252114153e-4) * t73994 + t76816 - t76817 + F::new(0.72714524817717142305e-5) * t74008 + t76820;
    t80022
}
