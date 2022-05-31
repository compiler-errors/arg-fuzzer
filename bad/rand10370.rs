
        struct S1;
        struct S2;
        struct S3;
        struct S4;
        struct S5;
        struct S6;
        struct S7;
        struct S8;
        
        fn foo10370(_: S4, _: S5) {}
        
        fn test10370() { foo10370(S1, S3, S4, S6, S7, S8); }
    