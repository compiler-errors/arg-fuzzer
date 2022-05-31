
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5033(_: S2, _: S5, _: S8) {}
        
        fn test5033() { foo5033(S5, S3, S2, S5, S5); }
    