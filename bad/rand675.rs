
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo675(_: S2, _: S6, _: S5, _: S5) {}
        
        fn test675() { foo675(S2, S3, S4, S6, S7, S8); }
    