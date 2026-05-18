//! MGGA_X_RLDA vxc pol kernel.
//!
//! Auto-translated from `libxc-master/src/maple2c/mgga_exc/mgga_x_rlda.c`.
//! Preserves exact maple2c variable names and FP operation order.

#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT4, M_CBRTPI, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case)]
#[cube(launch_unchecked)]
pub fn mgga_x_rlda_vxc_pol(
    rho: &Array<f64>,
    sigma: &Array<f64>,
    lapl: &Array<f64>,
    tau: &Array<f64>,
    zk: &mut Array<f64>,
    vrho: &mut Array<f64>,
    vsigma: &mut Array<f64>,
    vlapl: &mut Array<f64>,
    vtau: &mut Array<f64>,
    param_prefactor: f64,
    dens_threshold: f64,
    zeta_threshold: f64,
) {
    let ip = ABSOLUTE_POS;
    if ip < zk.len() {
        let rho0 = rho[ip * 2];
        let rho1 = rho[ip * 2 + 1];
        let sigma0 = sigma[ip * 3];
        let sigma1 = sigma[ip * 3 + 1];
        let sigma2 = sigma[ip * 3 + 2];
        let lapl0 = lapl[ip * 2];
        let lapl1 = lapl[ip * 2 + 1];
        let tau0 = tau[ip * 2];
        let tau1 = tau[ip * 2 + 1];
        let t2 = rho0 <= dens_threshold;
        let t3 = M_CBRTPI;
        let t4 = t3 * t3;
        let t5 = rho0 + rho1;
        let t6 = 1.0 / t5;
        let t9 = 2.0 * rho0 * t6 <= zeta_threshold;
        let t10 = zeta_threshold - 1.0;
        let t13 = 2.0 * rho1 * t6 <= zeta_threshold;
        let t14 = -t10;
        let t15 = rho0 - rho1;
        let t17 = piecewise5::<f64>(t9, t10, t13, t14, t15 * t6);
        let t18 = 1.0 + t17;
        let t19 = t18 <= zeta_threshold;
        let t20 = pow_1_3::<f64>(zeta_threshold);
        let t21 = t20 * zeta_threshold;
        let t22 = pow_1_3::<f64>(t18);
        let t24 = piecewise3::<f64>(t19, t21, t22 * t18);
        let t25 = t4 * t24;
        let t26 = pow_1_3::<f64>(t5);
        let t29 = pow_1_3::<f64>(1.0 / M_PI);
        let t30 = 1.0 / t29;
        let t31 = param_prefactor * t30;
        let t32 = M_CBRT4;
        let t33 = pow_1_3::<f64>(rho0);
        let t34 = t33 * t33;
        let t36 = 1.0 / t34 / rho0;
        let t41 = 2.0 * tau0 * t36 - lapl0 * t36 / 4.0;
        let t44 = t31 * t32 / t41;
        let t47 = piecewise3::<f64>(t2, 0.0, -15.0 / 16.0 * t25 * t26 * t44);
        let t48 = rho1 <= dens_threshold;
        let t49 = -t15;
        let t51 = piecewise5::<f64>(t13, t10, t9, t14, t49 * t6);
        let t52 = 1.0 + t51;
        let t53 = t52 <= zeta_threshold;
        let t54 = pow_1_3::<f64>(t52);
        let t56 = piecewise3::<f64>(t53, t21, t54 * t52);
        let t57 = t4 * t56;
        let t59 = pow_1_3::<f64>(rho1);
        let t60 = t59 * t59;
        let t62 = 1.0 / t60 / rho1;
        let t67 = 2.0 * tau1 * t62 - lapl1 * t62 / 4.0;
        let t70 = t31 * t32 / t67;
        let t73 = piecewise3::<f64>(t48, 0.0, -15.0 / 16.0 * t57 * t26 * t70);
        let tzk0 = t47 + t73;
        zk[ip] += tzk0;
        let t74 = t5 * t5;
        let t75 = 1.0 / t74;
        let t76 = t15 * t75;
        let t78 = piecewise5::<f64>(t9, 0.0, t13, 0.0, t6 - t76);
        let t81 = piecewise3::<f64>(t19, 0.0, 4.0 / 3.0 * t22 * t78);
        let t82 = t4 * t81;
        let t86 = t26 * t26;
        let t87 = 1.0 / t86;
        let t90 = 5.0 / 16.0 * t25 * t87 * t44;
        let t91 = t26 * param_prefactor;
        let t92 = t25 * t91;
        let t93 = t30 * t32;
        let t94 = t41 * t41;
        let t95 = 1.0 / t94;
        let t96 = rho0 * rho0;
        let t98 = 1.0 / t34 / t96;
        let t103 = -10.0 / 3.0 * tau0 * t98 + 5.0 / 12.0 * lapl0 * t98;
        let t105 = t93 * t95 * t103;
        let t109 = piecewise3::<f64>(t2, 0.0, -15.0 / 16.0 * t82 * t26 * t44 - t90 + 15.0 / 16.0 * t92 * t105);
        let t110 = t49 * t75;
        let t112 = piecewise5::<f64>(t13, 0.0, t9, 0.0, -t6 - t110);
        let t115 = piecewise3::<f64>(t53, 0.0, 4.0 / 3.0 * t54 * t112);
        let t116 = t4 * t115;
        let t122 = 5.0 / 16.0 * t57 * t87 * t70;
        let t124 = piecewise3::<f64>(t48, 0.0, -15.0 / 16.0 * t116 * t26 * t70 - t122);
        let tvrho0 = t47 + t73 + t5 * (t109 + t124);
        vrho[ip * 2] += tvrho0;
        let t128 = piecewise5::<f64>(t9, 0.0, t13, 0.0, -t6 - t76);
        let t131 = piecewise3::<f64>(t19, 0.0, 4.0 / 3.0 * t22 * t128);
        let t132 = t4 * t131;
        let t137 = piecewise3::<f64>(t2, 0.0, -15.0 / 16.0 * t132 * t26 * t44 - t90);
        let t139 = piecewise5::<f64>(t13, 0.0, t9, 0.0, t6 - t110);
        let t142 = piecewise3::<f64>(t53, 0.0, 4.0 / 3.0 * t54 * t139);
        let t143 = t4 * t142;
        let t147 = t57 * t91;
        let t148 = t67 * t67;
        let t149 = 1.0 / t148;
        let t150 = rho1 * rho1;
        let t152 = 1.0 / t60 / t150;
        let t157 = -10.0 / 3.0 * tau1 * t152 + 5.0 / 12.0 * lapl1 * t152;
        let t159 = t93 * t149 * t157;
        let t163 = piecewise3::<f64>(t48, 0.0, -15.0 / 16.0 * t143 * t26 * t70 - t122 + 15.0 / 16.0 * t147 * t159);
        let tvrho1 = t47 + t73 + t5 * (t137 + t163);
        vrho[ip * 2 + 1] += tvrho1;
        let tvsigma0 = 0.0;
        vsigma[ip * 3] += tvsigma0;
        let tvsigma1 = 0.0;
        vsigma[ip * 3 + 1] += tvsigma1;
        let tvsigma2 = 0.0;
        vsigma[ip * 3 + 2] += tvsigma2;
        let t167 = t93 * t95 * t36;
        let t168 = t92 * t167;
        let t170 = piecewise3::<f64>(t2, 0.0, -15.0 / 64.0 * t168);
        let tvlapl0 = t5 * t170;
        vlapl[ip * 2] += tvlapl0;
        let t172 = t93 * t149 * t62;
        let t173 = t147 * t172;
        let t175 = piecewise3::<f64>(t48, 0.0, -15.0 / 64.0 * t173);
        let tvlapl1 = t5 * t175;
        vlapl[ip * 2 + 1] += tvlapl1;
        let t177 = piecewise3::<f64>(t2, 0.0, 15.0 / 8.0 * t168);
        let tvtau0 = t5 * t177;
        vtau[ip * 2] += tvtau0;
        let t179 = piecewise3::<f64>(t48, 0.0, 15.0 / 8.0 * t173);
        let tvtau1 = t5 * t179;
        vtau[ip * 2 + 1] += tvtau1;
    }
}
