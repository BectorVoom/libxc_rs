//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1164/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1164<F: Float>(t7284: F, t7348: F, t24637: F, t8866: F, t32524: F, t85639: F, t1202: F, t32447: F, t3502: F, t483: F, t32448: F, t3523: F) -> (F, F, F, F, F, F) {
    let t117926 = t7284 * t7348;
    let t117930 = t8866 * t24637;
    let t117934 = t85639 * t32524;
    let t117949 = t1202 * t32447;
    let t117954 = t3502 * t483;
    let t117963 = t32448 * t3523;
    (t117926, t117930, t117934, t117949, t117954, t117963)
}
