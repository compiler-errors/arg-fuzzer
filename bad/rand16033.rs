
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo16033(_: S2, _: S3, _: S6, _: S7) {}
        
        fn test16033() { foo16033(S1, S7, S5, S0, S3, S0, S4); }
    