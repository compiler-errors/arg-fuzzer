
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo87(_: S6, _: S8, _: S2) {}
        
        fn test87() { foo87(S4, S7, S8); }
    