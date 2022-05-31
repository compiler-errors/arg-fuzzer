
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo500(_: S1, _: S4, _: S6) {}
        
        fn test500() { foo500(S2, S7, S5, S8, S1, S8, S5); }
    