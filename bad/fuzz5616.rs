
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo5616(_: S1, _: S5, _: S6, _: S7) {}
        
        fn test5616() { foo5616(S1, S4, S5, S6, S8); }
    