
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo821(_: S2, _: S5, _: S6, _: S7, _: S8) {}
        
        fn test821() { foo821(S3, S3, S7, S5, S5, S1, S7, S3); }
    