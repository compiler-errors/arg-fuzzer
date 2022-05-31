
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo13463(_: S1, _: S4, _: S5) {}
        
        fn test13463() { foo13463(S8, S7, S4, S3, S2); }
    