
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo82(_: S5, _: S7, _: S4, _: S8, _: S5, _: S4, _: S4, _: S7) {}
        
        fn test82() { foo82(S1, S2, S3, S6, S8); }
    