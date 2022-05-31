
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo4471(_: S5, _: S6, _: S7) {}
        
        fn test4471() { foo4471(S1, S3, S4, S5, S6, S8); }
    