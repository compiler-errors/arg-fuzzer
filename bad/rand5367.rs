
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5367(_: S1, _: S3) {}
        
        fn test5367() { foo5367(S2, S1, S2, S0, S5, S7); }
    