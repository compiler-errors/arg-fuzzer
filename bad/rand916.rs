
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo916(_: S2, _: S6) {}
        
        fn test916() { foo916(S1, S1, S3, S2, S1, S3); }
    