
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5196(_: S2, _: S4, _: S8, _: S2, _: S8) {}
        
        fn test5196() { foo5196(S6, S2, S3, S5, S4, S1); }
    